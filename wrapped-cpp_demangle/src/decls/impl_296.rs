macro_rules! deps {
    () => {
        ArgScope!();
        Result!();
        TemplateArg!();
        Error!();
        Type!();
        LeafName!();
        TemplateArgs!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < 'a > ArgScope < 'a , 'a > for WellKnownComponent { fn leaf_name (& 'a self) -> Result < LeafName < 'a > > { Ok (LeafName :: WellKnownComponent (self)) } fn get_template_arg (& 'a self , _ : usize) -> Result < (& 'a TemplateArg , & 'a TemplateArgs) > { Err (error :: Error :: BadTemplateArgReference) } fn get_function_arg (& 'a self , _ : usize) -> Result < & 'a Type > { Err (error :: Error :: BadFunctionArgReference) } }
    };
}

impl_296!()