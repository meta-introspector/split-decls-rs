macro_rules! deps {
    () => {
        TraitItemVFn!();
        MockableStruct!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Parse for MockableStruct { fn parse (input : ParseStream) -> syn :: parse :: Result < Self > { let attrs = input . call (syn :: Attribute :: parse_outer) ? ; let vis : syn :: Visibility = input . parse () ? ; let original_name : syn :: Ident = input . parse () ? ; let mut generics : syn :: Generics = input . parse () ? ; let wc : Option < syn :: WhereClause > = input . parse () ? ; generics . where_clause = wc ; let name = gen_mock_ident (& original_name) ; let impl_content ; let _brace_token = braced ! (impl_content in input) ; let mut consts = Vec :: new () ; let mut methods = Vec :: new () ; while ! impl_content . is_empty () { let item : ImplItem = impl_content . parse () ? ; match item { ImplItem :: Verbatim (ts) => { let tivf : TraitItemVFn = parse2 (ts) ? ; let mut iim = tif2iif (tivf . tif , & tivf . vis) ; mockable_method (& mut iim , & name , & generics) ; methods . push (iim) ; } ImplItem :: Const (iic) => consts . push (iic) , _ => { return Err (input . error ("Unsupported in this context")) ; } } } let mut impls = Vec :: new () ; while ! input . is_empty () { let item : Item = input . parse () ? ; match item { Item :: Impl (mut ii) => { for item in ii . items . iter_mut () { if let ImplItem :: Verbatim (ts) = item { let tif : TraitItemFn = parse2 (ts . clone ()) . unwrap () ; let iim = tif2iif (tif , & Visibility :: Inherited) ; * item = ImplItem :: Fn (iim) ; } } impls . push (mockable_item_impl (ii , & name , & generics)) ; } _ => return Err (input . error ("Unsupported in this context")) , } } Ok (MockableStruct { attrs , consts , generics , methods , name , vis , impls }) } }
    };
}

impl_106!();