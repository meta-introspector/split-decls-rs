macro_rules! deps {
    () => {
        Stream!();
        StyledStr!();
        ColorChoice!();
    };
}

macro_rules! Colorizer {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct Colorizer { stream : Stream , # [allow (unused)] color_when : ColorChoice , content : StyledStr , }
    };
}

Colorizer!()