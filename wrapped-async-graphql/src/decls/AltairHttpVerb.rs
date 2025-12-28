macro_rules! AltairHttpVerb {
    () => {
        # [doc = " Altair supported HTTP verbs"] # [derive (Serialize , Deserialize , JsonSchema)] # [allow (missing_docs)] pub enum AltairHttpVerb { POST , GET , PUT , DELETE , }
    };
}

AltairHttpVerb!()