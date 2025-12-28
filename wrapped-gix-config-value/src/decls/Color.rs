macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! Color {
    () => {
        deps!();
        # [doc = " Any value that may contain a foreground color, background color, a"] # [doc = " collection of color (text) modifiers, or a combination of any of the"] # [doc = " aforementioned values, like `red` or `brightgreen`."] # [doc = ""] # [doc = " Note that `git-config` allows color values to simply be a collection of"] # [doc = " [`color::Attribute`]s, and does not require a [`color::Name`] for either the"] # [doc = " foreground or background color."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug , Default)] pub struct Color { # [doc = " A provided foreground color"] pub foreground : Option < color :: Name > , # [doc = " A provided background color"] pub background : Option < color :: Name > , # [doc = " A potentially empty set of text attributes"] pub attributes : color :: Attribute , }
    };
}

Color!()