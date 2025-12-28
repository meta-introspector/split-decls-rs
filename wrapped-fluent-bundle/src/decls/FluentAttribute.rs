macro_rules! deps {
    () => {
        FluentMessage!();
        FluentBundle!();
    };
}

macro_rules! FluentAttribute {
    () => {
        deps!();
        # [doc = " [`FluentAttribute`] is a component of a compound [`FluentMessage`]."] # [doc = ""] # [doc = " It represents a key-value pair providing a translation of a component"] # [doc = " of a user interface widget localized by the given message."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_bundle::{FluentResource, FluentBundle};"] # [doc = ""] # [doc = " let source = r#\""] # [doc = ""] # [doc = " confirm-modal = Are you sure?"] # [doc = "     .confirm = Yes"] # [doc = "     .cancel = No"] # [doc = "     .tooltip = Closing the window will lose all unsaved data."] # [doc = ""] # [doc = " \"#;"] # [doc = ""] # [doc = " let resource = FluentResource::try_new(source.to_string())"] # [doc = "     .expect(\"Failed to parse the resource.\");"] # [doc = ""] # [doc = " let mut bundle = FluentBundle::default();"] # [doc = " bundle.add_resource(resource)"] # [doc = "     .expect(\"Failed to add a resource.\");"] # [doc = ""] # [doc = " let msg = bundle.get_message(\"confirm-modal\")"] # [doc = "     .expect(\"Failed to retrieve a message.\");"] # [doc = ""] # [doc = " let mut err = vec![];"] # [doc = ""] # [doc = " let attributes = msg.attributes().map(|attr| {"] # [doc = "     bundle.format_pattern(attr.value(), None, &mut err)"] # [doc = " }).collect::<Vec<_>>();"] # [doc = ""] # [doc = " assert_eq!(attributes[0], \"Yes\");"] # [doc = " assert_eq!(attributes[1], \"No\");"] # [doc = " assert_eq!(attributes[2], \"Closing the window will lose all unsaved data.\");"] # [doc = " ```"] # [derive (Debug , PartialEq)] pub struct FluentAttribute < 'm > { node : & 'm ast :: Attribute < & 'm str > , }
    };
}

FluentAttribute!()