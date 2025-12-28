macro_rules! deps {
    () => {
        Directive!();
        InputValue!();
        Object!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        # [doc = " A Directive provides a way to describe alternate runtime execution and type"] # [doc = " validation behavior in a GraphQL document."] # [doc = ""] # [doc = " In some cases, you need to provide options to alter GraphQL's execution"] # [doc = " behavior in ways field arguments will not suffice, such as conditionally"] # [doc = " including or skipping a field. Directives provide this by describing"] # [doc = " additional information to the executor."] # [Object (internal , name = "__Directive")] impl < 'a > __Directive < 'a > { # [inline] async fn name (& self) -> & str { & self . directive . name } # [inline] async fn description (& self) -> Option < & str > { self . directive . description . as_deref () } # [inline] async fn locations (& self) -> & Vec < __DirectiveLocation > { & self . directive . locations } async fn args (& self , # [graphql (default = false)] include_deprecated : bool ,) -> Vec < __InputValue < 'a > > { self . directive . args . values () . filter (| input_value | include_deprecated || ! input_value . deprecation . is_deprecated ()) . map (| input_value | __InputValue { registry : self . registry , visible_types : self . visible_types , input_value , }) . collect () } # [inline] async fn is_repeatable (& self) -> bool { self . directive . is_repeatable } }
    };
}

impl_79!()