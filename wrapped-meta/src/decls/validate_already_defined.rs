macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! validate_already_defined {
    () => {
        deps!();
        # [doc = " Validates that the given `definitions` do not contain any duplicate rules."] # [allow (clippy :: ptr_arg)] pub fn validate_already_defined (definitions : & Vec < Span < '_ > >) -> Vec < Error < Rule > > { let mut errors = vec ! [] ; let mut defined = HashSet :: new () ; for definition in definitions { let name = definition . as_str () ; if defined . contains (& name) { errors . push (Error :: new_from_span (ErrorVariant :: CustomError { message : format ! ("rule {} already defined" , name) , } , * definition ,)) } else { defined . insert (name) ; } } errors }
    };
}

validate_already_defined!()