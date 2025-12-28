macro_rules! deps {
    () => {
        FluentValue!();
        FluentResource!();
        FluentBundle!();
    };
}

macro_rules! FluentArgs {
    () => {
        deps!();
        # [doc = " Fluent messages can use arguments in order to programmatically add values to a"] # [doc = " translated string. For instance, in a localized application you may wish to display"] # [doc = " a user's email count. This could be done with the following message."] # [doc = ""] # [doc = " `msg-key = Hello, { $user }. You have { $emailCount } messages.`"] # [doc = ""] # [doc = " Here `$user` and `$emailCount` are the arguments, which can be filled with values."] # [doc = ""] # [doc = " The [`FluentArgs`] struct is the map from the argument name (for example `$user`) to"] # [doc = " the argument value (for example \"John\".) The logic to apply these to write these"] # [doc = " to messages is elsewhere, this struct just stores the value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};"] # [doc = ""] # [doc = " let mut args = FluentArgs::new();"] # [doc = " args.set(\"user\", \"John\");"] # [doc = " args.set(\"emailCount\", 5);"] # [doc = ""] # [doc = " let res = FluentResource::try_new(r#\""] # [doc = ""] # [doc = " msg-key = Hello, { $user }. You have { $emailCount } messages."] # [doc = ""] # [doc = " \"#.to_string())"] # [doc = "     .expect(\"Failed to parse FTL.\");"] # [doc = ""] # [doc = " let mut bundle = FluentBundle::default();"] # [doc = ""] # [doc = " // For this example, we'll turn on BiDi support."] # [doc = " // Please, be careful when doing it, it's a risky move."] # [doc = " bundle.set_use_isolating(false);"] # [doc = ""] # [doc = " bundle.add_resource(res)"] # [doc = "     .expect(\"Failed to add a resource.\");"] # [doc = ""] # [doc = " let mut err = vec![];"] # [doc = ""] # [doc = " let msg = bundle.get_message(\"msg-key\")"] # [doc = "     .expect(\"Failed to retrieve a message.\");"] # [doc = " let value = msg.value()"] # [doc = "     .expect(\"Failed to retrieve a value.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     bundle.format_pattern(value, Some(&args), &mut err),"] # [doc = "     \"Hello, John. You have 5 messages.\""] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Default)] pub struct FluentArgs < 'args > (Vec < (Cow < 'args , str > , FluentValue < 'args >) >) ;
    };
}

FluentArgs!();