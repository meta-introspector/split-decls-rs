macro_rules! AttributeTemplate {
    () => {
        # [doc = " A template that the attribute input must match."] # [doc = " Only top-level shape (`#[attr]` vs `#[attr(...)]` vs `#[attr = ...]`) is considered now."] # [derive (Clone , Copy)] pub struct AttributeTemplate { pub word : bool , pub list : Option < & 'static str > , pub name_value_str : Option < & 'static str > , }
    };
}

AttributeTemplate!();