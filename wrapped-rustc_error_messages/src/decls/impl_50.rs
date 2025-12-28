macro_rules! deps {
    () => {
        DiagMessage!();
        SubdiagMessage!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        # [doc = " Translating *into* a subdiagnostic message from a diagnostic message is a little strange - but"] # [doc = " the subdiagnostic functions (e.g. `span_label`) take a `SubdiagMessage` and the"] # [doc = " subdiagnostic derive refers to typed identifiers that are `DiagMessage`s, so need to be"] # [doc = " able to convert between these, as much as they'll be converted back into `DiagMessage`"] # [doc = " using `with_subdiagnostic_message` eventually. Don't use this other than for the derive."] impl From < DiagMessage > for SubdiagMessage { fn from (val : DiagMessage) -> Self { match val { DiagMessage :: Str (s) => SubdiagMessage :: Str (s) , DiagMessage :: Translated (s) => SubdiagMessage :: Translated (s) , DiagMessage :: FluentIdentifier (id , None) => SubdiagMessage :: FluentIdentifier (id) , DiagMessage :: FluentIdentifier (_ , Some (attr)) => SubdiagMessage :: FluentAttr (attr) , } } }
    };
}

impl_50!();