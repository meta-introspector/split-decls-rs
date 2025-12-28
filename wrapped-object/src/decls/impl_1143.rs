macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! impl_1143 {
    () => {
        deps!();
        impl < 'data > Dynamic < 'data > { # [doc = " The `d_tag` field in the dynamic entry."] # [doc = ""] # [doc = " One of the `DT_*` values."] pub fn tag (& self) -> u32 { match self { Dynamic :: Auto { tag } => * tag , Dynamic :: Integer { tag , .. } => * tag , Dynamic :: String { tag , .. } => * tag , } } }
    };
}

impl_1143!();