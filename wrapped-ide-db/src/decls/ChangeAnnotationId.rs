macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
    };
}

macro_rules! ChangeAnnotationId {
    () => {
        deps!();
        # [doc = " An annotation ID associated with an indel, to describe changes."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , UpmapFromRaFixture)] pub struct ChangeAnnotationId (u32) ;
    };
}

ChangeAnnotationId!()