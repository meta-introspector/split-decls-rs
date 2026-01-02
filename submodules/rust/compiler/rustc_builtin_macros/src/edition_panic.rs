mkuse!{use rustc_ast :: token :: Delimiter ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpan , TokenStream } ;}
mkuse!{use rustc_ast :: * ;}
mkuse!{use rustc_expand :: base :: * ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { Span , sym } ;}

macro_rules! expand_panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_panic in module {}", module_path!());
    };
}

mkfn!{
    expand_panic_introspect!();
    # [doc = " This expands to either"] # [doc = " - `$crate::panic::panic_2015!(...)` or"] # [doc = " - `$crate::panic::panic_2021!(...)`"] # [doc = " depending on the edition."] # [doc = ""] # [doc = " This is used for both std::panic!() and core::panic!()."] # [doc = ""] # [doc = " `$crate` will refer to either the `std` or `core` crate depending on which"] # [doc = " one we're expanding from."] pub (crate) fn expand_panic < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let mac = if use_panic_2021 (sp) { sym :: panic_2021 } else { sym :: panic_2015 } ; expand (mac , cx , sp , tts) }
}

macro_rules! expand_unreachable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_unreachable in module {}", module_path!());
    };
}

mkfn!{
    expand_unreachable_introspect!();
    # [doc = " This expands to either"] # [doc = " - `$crate::panic::unreachable_2015!(...)` or"] # [doc = " - `$crate::panic::unreachable_2021!(...)`"] # [doc = " depending on the edition."] pub (crate) fn expand_unreachable < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let mac = if use_panic_2021 (sp) { sym :: unreachable_2021 } else { sym :: unreachable_2015 } ; expand (mac , cx , sp , tts) }
}

macro_rules! expand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand in module {}", module_path!());
    };
}

mkfn!{
    expand_introspect!();
    fn expand < 'cx > (mac : rustc_span :: Symbol , cx : & 'cx ExtCtxt < '_ > , sp : Span , tts : TokenStream ,) -> MacroExpanderResult < 'cx > { let sp = cx . with_call_site_ctxt (sp) ; ExpandResult :: Ready (MacEager :: expr (cx . expr (sp , ExprKind :: MacCall (Box :: new (MacCall { path : Path { span : sp , segments : cx . std_path (& [sym :: panic , mac]) . into_iter () . map (| ident | PathSegment :: from_ident (ident)) . collect () , tokens : None , } , args : Box :: new (DelimArgs { dspan : DelimSpan :: from_single (sp) , delim : Delimiter :: Parenthesis , tokens : tts , }) , })) ,) ,)) }
}

macro_rules! use_panic_2021_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function use_panic_2021 in module {}", module_path!());
    };
}

mkfn!{
    use_panic_2021_introspect!();
    pub (crate) fn use_panic_2021 (mut span : Span) -> bool { loop { let expn = span . ctxt () . outer_expn_data () ; if let Some (features) = expn . allow_internal_unstable && features . contains (& sym :: edition_panic) { span = expn . call_site ; continue ; } break expn . edition >= Edition :: Edition2021 ; } }
}