macro_rules! IntrospectionMode {
    () => {
        # [doc = " Introspection mode"] # [derive (Debug , Copy , Clone , PartialEq , Eq , Default)] pub enum IntrospectionMode { # [doc = " Introspection only"] IntrospectionOnly , # [doc = " Enables introspection"] # [default] Enabled , # [doc = " Disables introspection"] Disabled , }
    };
}

IntrospectionMode!();