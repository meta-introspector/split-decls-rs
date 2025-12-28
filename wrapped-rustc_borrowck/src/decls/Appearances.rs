macro_rules! deps {
    () => {
        Appearance!();
    };
}

macro_rules! Appearances {
    () => {
        deps!();
        type Appearances = IndexVec < AppearanceIndex , Appearance > ;
    };
}

Appearances!();