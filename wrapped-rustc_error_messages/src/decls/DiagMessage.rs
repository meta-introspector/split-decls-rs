macro_rules! deps {
    () => {
        FluentId!();
    };
}

macro_rules! DiagMessage {
    () => {
        deps!();
        # [doc = " Abstraction over a message in a diagnostic to support both translatable and non-translatable"] # [doc = " diagnostic messages."] # [doc = ""] # [doc = " Intended to be removed once diagnostics are entirely translatable."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Encodable , Decodable)] # [rustc_diagnostic_item = "DiagMessage"] pub enum DiagMessage { # [doc = " Non-translatable diagnostic message."] Str (Cow < 'static , str >) , # [doc = " Translatable message which has been already translated."] # [doc = ""] # [doc = " Some diagnostics have repeated subdiagnostics where the same interpolated variables would"] # [doc = " be instantiated multiple times with different values. These subdiagnostics' messages"] # [doc = " are translated when they are added to the parent diagnostic, producing this variant of"] # [doc = " `DiagMessage`."] Translated (Cow < 'static , str >) , # [doc = " Identifier for a Fluent message (with optional attribute) corresponding to the diagnostic"] # [doc = " message. Yet to be translated."] # [doc = ""] # [doc = " <https://projectfluent.org/fluent/guide/hello.html>"] # [doc = " <https://projectfluent.org/fluent/guide/attributes.html>"] FluentIdentifier (FluentId , Option < FluentId >) , }
    };
}

DiagMessage!()