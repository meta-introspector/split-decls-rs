macro_rules! ClassLayout {
    () => {
        pub struct ClassLayout { pub PackingSize : u16 , pub ClassSize : u32 , pub Parent : u32 , }
    };
}

ClassLayout!()