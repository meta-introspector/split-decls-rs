macro_rules! deps {
    () => {
        Error!();
        TemplateParam!();
        Result!();
        ArgScopeStack!();
        TemplateArg!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl TemplateParam { fn resolve < 'subs , 'prev > (& 'subs self , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> :: core :: result :: Result < & 'subs TemplateArg , fmt :: Error > { scope . get_template_arg (self . 0) . map_err (| e | { log ! ("Error obtaining template argument: {}" , e) ; fmt :: Error }) . map (| v | v . 0) } }
    };
}

impl_223!()