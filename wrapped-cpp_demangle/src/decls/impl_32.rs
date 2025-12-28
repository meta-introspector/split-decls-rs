macro_rules! deps {
    () => {
        Type!();
        ArgScope!();
        Error!();
        ArgScopeStack!();
        Result!();
        TemplateArg!();
        TemplateArgs!();
        LeafName!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [doc = " A stack of `ArgScope`s is itself an `ArgScope`!"] impl < 'prev , 'subs > ArgScope < 'prev , 'subs > for Option < ArgScopeStack < 'prev , 'subs > > { fn leaf_name (& 'prev self) -> Result < LeafName < 'subs > > { let mut scope = self . as_ref () ; while let Some (s) = scope { if let Ok (c) = s . item . leaf_name () { return Ok (c) ; } scope = s . prev ; } Err (error :: Error :: BadLeafNameReference) } fn get_template_arg (& 'prev self , idx : usize ,) -> Result < (& 'subs TemplateArg , & 'subs TemplateArgs) > { let mut scope = self . as_ref () ; while let Some (s) = scope { if let Ok ((arg , args)) = s . item . get_template_arg (idx) { if let Some ((in_idx , in_args)) = s . in_arg { if args as * const TemplateArgs == in_args as * const TemplateArgs && in_idx <= idx { return Err (error :: Error :: ForwardTemplateArgReference) ; } } return Ok ((arg , args)) ; } scope = s . prev ; } Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'prev self , idx : usize) -> Result < & 'subs Type > { let mut scope = self . as_ref () ; while let Some (s) = scope { if let Ok (arg) = s . item . get_function_arg (idx) { return Ok (arg) ; } scope = s . prev ; } Err (error :: Error :: BadFunctionArgReference) } }
    };
}

impl_32!();