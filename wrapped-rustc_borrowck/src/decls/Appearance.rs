macro_rules! Appearance {
    () => {
        struct Appearance { point_index : PointIndex , next : Option < AppearanceIndex > , }
    };
}

Appearance!()