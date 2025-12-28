macro_rules! deps {
    () => {
        Vertical!();
        Horizontal!();
    };
}

macro_rules! Position {
    () => {
        deps!();
        # [doc = " Position of the key"] # [derive (Clone , Copy)] pub enum Position { # [doc = " Inside the area surrounded by the four (Bottom X, Top X, Left Y and Right Y) axes"] Inside (Vertical , Horizontal) , # [doc = " Outside of that area"] Outside (Vertical , Horizontal) , }
    };
}

Position!()