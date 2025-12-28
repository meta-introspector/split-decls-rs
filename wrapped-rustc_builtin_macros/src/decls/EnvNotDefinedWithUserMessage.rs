macro_rules! EnvNotDefinedWithUserMessage {
    () => {
        pub (crate) struct EnvNotDefinedWithUserMessage { pub (crate) span : Span , pub (crate) msg_from_user : Symbol , }
    };
}

EnvNotDefinedWithUserMessage!();