macro_rules! deps {
    () => {
        MetaItemOrLitParser!();
        MetaItemListParserContext!();
        MetaItemListParser!();
        ShouldEmit!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'a > MetaItemListParser < 'a > { fn new < 'sess > (delim : & 'a DelimArgs , psess : & 'sess ParseSess , should_emit : ShouldEmit ,) -> Option < Self > { match MetaItemListParserContext :: parse (delim . tokens . clone () , psess , delim . dspan . entire () , should_emit ,) { Ok (s) => Some (s) , Err (e) => { should_emit . emit_err (e) ; None } } } # [doc = " Lets you pick and choose as what you want to parse each element in the list"] pub fn mixed (& self) -> impl Iterator < Item = & MetaItemOrLitParser < 'a > > { self . sub_parsers . iter () } pub fn len (& self) -> usize { self . sub_parsers . len () } pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns Some if the list contains only a single element."] # [doc = ""] # [doc = " Inside the Some is the parser to parse this single element."] pub fn single (& self) -> Option < & MetaItemOrLitParser < 'a > > { let mut iter = self . mixed () ; iter . next () . filter (| _ | iter . next () . is_none ()) } }
    };
}

impl_310!()