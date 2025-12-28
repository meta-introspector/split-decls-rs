macro_rules! deps {
    () => {
        Ty!();
        Method!();
    };
}

macro_rules! default_action {
    () => {
        deps!();
        fn default_action (field_type : & Type , span : Span) -> Method { let ty = Ty :: from_syn_ty (field_type) ; let args = match * ty { Ty :: Vec | Ty :: OptionVec | Ty :: VecVec | Ty :: OptionVecVec => { quote_spanned ! { span => clap :: ArgAction :: Append } } Ty :: Option | Ty :: OptionOption => { quote_spanned ! { span => clap :: ArgAction :: Set } } _ => { if is_simple_ty (field_type , "bool") { quote_spanned ! { span => clap :: ArgAction :: SetTrue } } else { quote_spanned ! { span => clap :: ArgAction :: Set } } } } ; let func = Ident :: new ("action" , span) ; Method :: new (func , args) }
    };
}

default_action!();