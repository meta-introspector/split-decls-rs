macro_rules! deps {
    () => {
        FluentBundle!();
        FluentAttribute!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'm > FluentAttribute < 'm > { # [doc = " Retrieves an id of an attribute."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use fluent_bundle::{FluentResource, FluentBundle};"] # [doc = " # let source = r#\""] # [doc = " # confirm-modal ="] # [doc = " #     .confirm = Yes"] # [doc = " # \"#;"] # [doc = " # let resource = FluentResource::try_new(source.to_string())"] # [doc = " #     .expect(\"Failed to parse the resource.\");"] # [doc = " # let mut bundle = FluentBundle::default();"] # [doc = " # bundle.add_resource(resource)"] # [doc = " #     .expect(\"Failed to add a resource.\");"] # [doc = " let msg = bundle.get_message(\"confirm-modal\")"] # [doc = "     .expect(\"Failed to retrieve a message.\");"] # [doc = ""] # [doc = " let attr1 = msg.attributes().next()"] # [doc = "     .expect(\"Failed to retrieve an attribute.\");"] # [doc = ""] # [doc = " assert_eq!(attr1.id(), \"confirm\");"] # [doc = " ```"] pub fn id (& self) -> & 'm str { self . node . id . name } # [doc = " Retrieves an value of an attribute."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use fluent_bundle::{FluentResource, FluentBundle};"] # [doc = " # let source = r#\""] # [doc = " # confirm-modal ="] # [doc = " #     .confirm = Yes"] # [doc = " # \"#;"] # [doc = " # let resource = FluentResource::try_new(source.to_string())"] # [doc = " #     .expect(\"Failed to parse the resource.\");"] # [doc = " # let mut bundle = FluentBundle::default();"] # [doc = " # bundle.add_resource(resource)"] # [doc = " #     .expect(\"Failed to add a resource.\");"] # [doc = " let msg = bundle.get_message(\"confirm-modal\")"] # [doc = "     .expect(\"Failed to retrieve a message.\");"] # [doc = ""] # [doc = " let attr1 = msg.attributes().next()"] # [doc = "     .expect(\"Failed to retrieve an attribute.\");"] # [doc = ""] # [doc = " let mut err = vec![];"] # [doc = ""] # [doc = " let value = attr1.value();"] # [doc = " assert_eq!("] # [doc = "     bundle.format_pattern(value, None, &mut err),"] # [doc = "     \"Yes\""] # [doc = " );"] # [doc = " ```"] pub fn value (& self) -> & 'm ast :: Pattern < & 'm str > { & self . node . value } }
    };
}

impl_36!()