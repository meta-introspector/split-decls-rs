macro_rules! deps {
    () => {
        Writer!();
        Section!();
        SectionId!();
    };
}

macro_rules! define_section {
    () => {
        deps!();
        macro_rules ! define_section { ($ name : ident , $ offset : ident , $ docs : expr) => { # [doc =$ docs] # [derive (Debug , Default)] pub struct $ name < W : Writer > (pub W) ; impl < W : Writer > $ name < W > { # [doc = " Return the offset of the next write."] pub fn offset (& self) -> $ offset { $ offset (self . len ()) } } impl < W : Writer > From < W > for $ name < W > { # [inline] fn from (w : W) -> Self { $ name (w) } } impl < W : Writer > Deref for $ name < W > { type Target = W ; # [inline] fn deref (& self) -> & W { & self . 0 } } impl < W : Writer > DerefMut for $ name < W > { # [inline] fn deref_mut (& mut self) -> & mut W { & mut self . 0 } } impl < W : Writer > Section < W > for $ name < W > { # [inline] fn id (& self) -> SectionId { SectionId ::$ name } } } ; }
    };
}

define_section!()