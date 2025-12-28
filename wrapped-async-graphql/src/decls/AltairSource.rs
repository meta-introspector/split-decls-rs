macro_rules! AltairSource {
    () => {
        # [doc = " A builder for constructing an Altair HTML page."] # [derive (Default , Serialize)] pub struct AltairSource < 'a > { # [serde (default , skip_serializing_if = "Option::is_none")] title : Option < & 'a str > , # [serde (default , skip_serializing_if = "Option::is_none")] options : Option < serde_json :: Value > , }
    };
}

AltairSource!();