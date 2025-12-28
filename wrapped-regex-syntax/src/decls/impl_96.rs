macro_rules! deps {
    () => {
        ClassSetUnion!();
        ClassSetItem!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl ClassSetUnion { # [doc = " Push a new item in this union."] # [doc = ""] # [doc = " The ending position of this union's span is updated to the ending"] # [doc = " position of the span of the item given. If the union is empty, then"] # [doc = " the starting position of this union is set to the starting position"] # [doc = " of this item."] # [doc = ""] # [doc = " In other words, if you only use this method to add items to a union"] # [doc = " and you set the spans on each item correctly, then you should never"] # [doc = " need to adjust the span of the union directly."] pub fn push (& mut self , item : ClassSetItem) { if self . items . is_empty () { self . span . start = item . span () . start ; } self . span . end = item . span () . end ; self . items . push (item) ; } # [doc = " Return this union as a character class set item."] # [doc = ""] # [doc = " If this union contains zero items, then an empty union is"] # [doc = " returned. If this concatenation contains exactly 1 item, then the"] # [doc = " corresponding item is returned. Otherwise, ClassSetItem::Union is"] # [doc = " returned."] pub fn into_item (mut self) -> ClassSetItem { match self . items . len () { 0 => ClassSetItem :: Empty (self . span) , 1 => self . items . pop () . unwrap () , _ => ClassSetItem :: Union (self) , } } }
    };
}

impl_96!()