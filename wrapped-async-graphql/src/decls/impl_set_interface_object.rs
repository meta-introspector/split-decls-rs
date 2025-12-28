macro_rules! impl_set_interface_object {
    () => {
        macro_rules ! impl_set_interface_object { () => { # [doc = " During composition, the fields of every `@interfaceObject` are added"] # [doc = " both to their corresponding interface definition and to all"] # [doc = " entity types that implement that interface."] # [doc = ""] # [doc = " Reference: <https://www.apollographql.com/docs/federation/federated-types/federated-directives/#interfaceobject>"] # [inline] pub fn interface_object (self) -> Self { Self { interface_object : true , .. self } } } ; }
    };
}

impl_set_interface_object!()