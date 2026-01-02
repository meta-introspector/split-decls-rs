mkuse!{use proc_macro2 :: Ident ;}
mkuse!{use quote :: quote ;}
mkuse!{use syn :: parse :: { Parse , ParseStream } ;}
mkuse!{use syn :: punctuated :: Punctuated ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: { Attribute , Generics , ImplItem , Pat , PatIdent , Path , Signature , Token , TraitItem , TraitItemConst , TraitItemFn , TraitItemMacro , TraitItemType , Type , Visibility , WhereClause , braced , parse_macro_input , } ;}

macro_rules! extension_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extension in module {}", module_path!());
    };
}

mkfn!{
    extension_introspect!();
    pub (crate) fn extension (attr : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let ExtensionAttr { vis , trait_ } = parse_macro_input ! (attr as ExtensionAttr) ; let Impl { attrs , generics , self_ty , items , wc } = parse_macro_input ! (input as Impl) ; let headers : Vec < _ > = items . iter () . map (| item | match item { ImplItem :: Fn (f) => TraitItem :: Fn (TraitItemFn { attrs : scrub_attrs (& f . attrs) , sig : scrub_header (f . sig . clone ()) , default : None , semi_token : Some (Token ! [;] (f . block . span ())) , }) , ImplItem :: Const (ct) => TraitItem :: Const (TraitItemConst { attrs : scrub_attrs (& ct . attrs) , const_token : ct . const_token , ident : ct . ident . clone () , generics : ct . generics . clone () , colon_token : ct . colon_token , ty : ct . ty . clone () , default : None , semi_token : ct . semi_token , }) , ImplItem :: Type (ty) => TraitItem :: Type (TraitItemType { attrs : scrub_attrs (& ty . attrs) , type_token : ty . type_token , ident : ty . ident . clone () , generics : ty . generics . clone () , colon_token : None , bounds : Punctuated :: new () , default : None , semi_token : ty . semi_token , }) , ImplItem :: Macro (mac) => TraitItem :: Macro (TraitItemMacro { attrs : scrub_attrs (& mac . attrs) , mac : mac . mac . clone () , semi_token : mac . semi_token , }) , ImplItem :: Verbatim (stream) => TraitItem :: Verbatim (stream . clone ()) , _ => unimplemented ! () , }) . collect () ; quote ! { # (# attrs) * # vis trait # trait_ { # (# headers) * } impl # generics # trait_ for # self_ty # wc { # (# items) * } } . into () }
}

macro_rules! scrub_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scrub_attrs in module {}", module_path!());
    };
}

mkfn!{
    scrub_attrs_introspect!();
    # [doc = " Only keep `#[doc]` attrs."] fn scrub_attrs (attrs : & [Attribute]) -> Vec < Attribute > { attrs . into_iter () . cloned () . filter (| attr | { let ident = & attr . path () . segments [0] . ident ; ident == "doc" || ident == "must_use" }) . collect () }
}

macro_rules! scrub_header_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function scrub_header in module {}", module_path!());
    };
}

mkfn!{
    scrub_header_introspect!();
    # [doc = " Scrub arguments so that they're valid for trait signatures."] fn scrub_header (mut sig : Signature) -> Signature { for (idx , input) in sig . inputs . iter_mut () . enumerate () { match input { syn :: FnArg :: Receiver (rcvr) => { if rcvr . reference . is_none () { rcvr . mutability . take () ; } } syn :: FnArg :: Typed (arg) => match & mut * arg . pat { Pat :: Ident (arg) => { arg . by_ref . take () ; arg . mutability . take () ; arg . subpat . take () ; } _ => { arg . pat = Box :: new (PatIdent { attrs : vec ! [] , by_ref : None , mutability : None , ident : Ident :: new (& format ! ("__arg{idx}") , arg . pat . span ()) , subpat : None , } . into () ,) } } , } } sig }
}
mkitem!{mkstruct!{struct ExtensionAttr { vis : Visibility , trait_ : Path , }}}
mkitem!{mkimpl!{impl Parse for ExtensionAttr { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let vis = input . parse () ? ; let _ : Token ! [trait] = input . parse () ? ; let trait_ = input . parse () ? ; Ok (ExtensionAttr { vis , trait_ }) } }}}
mkitem!{mkstruct!{struct Impl { attrs : Vec < Attribute > , generics : Generics , self_ty : Type , items : Vec < ImplItem > , wc : Option < WhereClause > , }}}
mkitem!{mkimpl!{impl Parse for Impl { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let _ : Token ! [impl] = input . parse () ? ; let generics = input . parse () ? ; let self_ty = input . parse () ? ; let wc = input . parse () ? ; let content ; let _brace_token = braced ! (content in input) ; let mut items = Vec :: new () ; while ! content . is_empty () { items . push (content . parse () ?) ; } Ok (Impl { attrs , generics , self_ty , items , wc }) } }}}