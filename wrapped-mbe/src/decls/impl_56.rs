macro_rules! deps {
    () => {
        MetaTemplate!();
        ExpandResult!();
        MatchedArmIndex!();
        ParseError!();
        Rule!();
        DeclarativeMacro!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl DeclarativeMacro { pub fn from_err (err : ParseError) -> DeclarativeMacro { DeclarativeMacro { rules : Box :: default () , err : Some (Box :: new (err)) } } # [doc = " The old, `macro_rules! m {}` flavor."] pub fn parse_macro_rules (tt : & tt :: TopSubtree < Span > , ctx_edition : impl Copy + Fn (SyntaxContext) -> Edition ,) -> DeclarativeMacro { let mut src = tt . iter () ; let mut rules = Vec :: new () ; let mut err = None ; while ! src . is_empty () { let rule = match Rule :: parse (ctx_edition , & mut src) { Ok (it) => it , Err (e) => { err = Some (Box :: new (e)) ; break ; } } ; rules . push (rule) ; if let Err (()) = src . expect_char (';') { if ! src . is_empty () { err = Some (Box :: new (ParseError :: expected ("expected `;`"))) ; } break ; } } for Rule { lhs , .. } in & rules { if let Err (e) = validate (lhs) { err = Some (Box :: new (e)) ; break ; } } DeclarativeMacro { rules : rules . into_boxed_slice () , err } } # [doc = " The new, unstable `macro m {}` flavor."] pub fn parse_macro2 (args : Option < & tt :: TopSubtree < Span > > , body : & tt :: TopSubtree < Span > , ctx_edition : impl Copy + Fn (SyntaxContext) -> Edition ,) -> DeclarativeMacro { let mut rules = Vec :: new () ; let mut err = None ; if let Some (args) = args { cov_mark :: hit ! (parse_macro_def_simple) ; let rule = (| | { let lhs = MetaTemplate :: parse_pattern (ctx_edition , args . iter ()) ? ; let rhs = MetaTemplate :: parse_template (ctx_edition , body . iter ()) ? ; Ok (crate :: Rule { lhs , rhs }) }) () ; match rule { Ok (rule) => rules . push (rule) , Err (e) => err = Some (Box :: new (e)) , } } else { cov_mark :: hit ! (parse_macro_def_rules) ; let mut src = body . iter () ; while ! src . is_empty () { let rule = match Rule :: parse (ctx_edition , & mut src) { Ok (it) => it , Err (e) => { err = Some (Box :: new (e)) ; break ; } } ; rules . push (rule) ; if let Err (()) = src . expect_any_char (& [';' , ',']) { if ! src . is_empty () { err = Some (Box :: new (ParseError :: expected ("expected `;` or `,` to delimit rules" ,))) ; } break ; } } } for Rule { lhs , .. } in & rules { if let Err (e) = validate (lhs) { err = Some (Box :: new (e)) ; break ; } } DeclarativeMacro { rules : rules . into_boxed_slice () , err } } pub fn err (& self) -> Option < & ParseError > { self . err . as_deref () } pub fn num_rules (& self) -> usize { self . rules . len () } pub fn expand (& self , tt : & tt :: TopSubtree < Span > , marker : impl Fn (& mut Span) + Copy , call_site : Span , def_site_edition : Edition ,) -> ExpandResult < (tt :: TopSubtree < Span > , MatchedArmIndex) > { expander :: expand_rules (& self . rules , tt , marker , call_site , def_site_edition) } }
    };
}

impl_56!();