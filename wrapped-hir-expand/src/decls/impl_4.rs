macro_rules! deps {
    () => {
        AttrId!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl AttrId { const INNER_ATTR_SET_BIT : u32 = 1 << 31 ; pub fn new (id : usize , is_inner : bool) -> Self { assert ! (id <= ! Self :: INNER_ATTR_SET_BIT as usize) ; let id = id as u32 ; Self { id : if is_inner { id | Self :: INNER_ATTR_SET_BIT } else { id } } } pub fn ast_index (& self) -> usize { (self . id & ! Self :: INNER_ATTR_SET_BIT) as usize } pub fn is_inner_attr (& self) -> bool { self . id & Self :: INNER_ATTR_SET_BIT != 0 } }
    };
}

impl_4!();