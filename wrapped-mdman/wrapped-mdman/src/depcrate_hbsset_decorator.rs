// Generated macro for set_decorator (function)
macro_rules! Depcrate_hbsset_decorator {
() => {
// Module: crate::hbs
// Provides: {"set_decorator"}
// Dependencies: {}
# [doc = " `{{*set var=value}}` decorator."] # [doc = ""] # [doc = " This sets a variable to a value within the template context."] fn set_decorator (d : & Decorator < '_ > , _ : & Handlebars < '_ > , _ctx : & Context , rc : & mut RenderContext < '_ , '_ > ,) -> Result < () , RenderError > { let data_to_set = d . hash () ; for (k , v) in data_to_set { set_in_context (rc , k , v . value () . clone ()) ; } Ok (()) }
};
}
