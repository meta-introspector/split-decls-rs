macro_rules! AttributeTemplate {
    () => {
        # [doc = " A template that the attribute input must match."] # [doc = " Only top-level shape (`#[attr]` vs `#[attr(...)]` vs `#[attr = ...]`) is considered now."] # [derive (Clone , Copy , Default)] pub struct AttributeTemplate { # [doc = " If `true`, the attribute is allowed to be a bare word like `#[test]`."] pub word : bool , # [doc = " If `Some`, the attribute is allowed to take a list of items like `#[allow(..)]`."] pub list : Option < & 'static [& 'static str] > , # [doc = " If non-empty, the attribute is allowed to take a list containing exactly"] # [doc = " one of the listed words, like `#[coverage(off)]`."] pub one_of : & 'static [Symbol] , # [doc = " If `Some`, the attribute is allowed to be a name/value pair where the"] # [doc = " value is a string, like `#[must_use = \"reason\"]`."] pub name_value_str : Option < & 'static [& 'static str] > , # [doc = " A link to the document for this attribute."] pub docs : Option < & 'static str > , }
    };
}

AttributeTemplate!();