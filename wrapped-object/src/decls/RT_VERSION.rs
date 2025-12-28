macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! RT_VERSION {
    () => {
        deps!();
        # [doc = " ID for: Version resource."] pub const RT_VERSION : u16 = 16 ;
    };
}

RT_VERSION!();