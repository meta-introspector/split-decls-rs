macro_rules! deps {
    () => {
        ExtendTarget!();
    };
}

macro_rules! impl_extend {
    () => {
        deps!();
        macro_rules ! impl_extend { ($ ($ from : ty => $ ($ to : ty) ,+;) *) => { $ ($ (const _ : () = assert ! (core :: mem :: size_of ::<$ from > () <= core :: mem :: size_of ::<$ to > () , concat ! ("cannot extend " , stringify ! ($ from) , " to " , stringify ! ($ to) , " because " , stringify ! ($ from) , " is larger than " , stringify ! ($ to))) ; impl sealed :: ExtendTargetSealed <$ to > for $ from { fn extend (self) -> $ to { self as _ } } impl ExtendTarget <$ to > for $ from { }) +) * } ; }
    };
}

impl_extend!()