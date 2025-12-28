macro_rules! deps {
    () => {
        StyledStr!();
        ColorChoice!();
        Stream!();
    };
}

macro_rules! Colorizer {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct Colorizer { stream : Stream , # [allow (unused)] color_when : ColorChoice , content : StyledStr , }
    };
}

Colorizer!();