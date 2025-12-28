macro_rules! deps {
    () => {
        GemEntry!();
    };
}

macro_rules! GemConfig {
    () => {
        deps!();
        # [derive (Debug , Deserialize)] pub struct GemConfig { pub gem : Vec < GemEntry > , }
    };
}

GemConfig!()