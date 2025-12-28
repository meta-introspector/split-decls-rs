macro_rules! deps {
    () => {
        RuleBuilder!();
        RawPattern!();
        ParsedRule!();
        SsrError!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl ParsedRule { fn new (pattern : & RawPattern , template : Option < & RawPattern > ,) -> Result < Vec < ParsedRule > , SsrError > { let raw_pattern = pattern . as_rust_code () ; let raw_template = template . map (| t | t . as_rust_code ()) ; let raw_template = raw_template . as_deref () ; let mut builder = RuleBuilder { placeholders_by_stand_in : pattern . placeholders_by_stand_in () , rules : Vec :: new () , } ; let raw_template_stmt = raw_template . map (fragments :: stmt) ; if let raw_template_expr @ Some (Ok (_)) = raw_template . map (fragments :: expr) { builder . try_add (fragments :: expr (& raw_pattern) , raw_template_expr) ; } else { builder . try_add (fragments :: expr (& raw_pattern) , raw_template_stmt . clone ()) ; } builder . try_add (fragments :: ty (& raw_pattern) , raw_template . map (fragments :: ty)) ; builder . try_add (fragments :: item (& raw_pattern) , raw_template . map (fragments :: item)) ; builder . try_add (fragments :: pat (& raw_pattern) , raw_template . map (fragments :: pat)) ; builder . try_add (fragments :: stmt (& raw_pattern) , raw_template_stmt) ; builder . build () } }
    };
}

impl_47!()