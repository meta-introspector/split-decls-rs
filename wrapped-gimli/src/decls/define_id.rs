macro_rules! deps {
    () => {
        BaseId!();
    };
}

macro_rules! define_id {
    () => {
        deps!();
        macro_rules ! define_id { ($ name : ident , $ docs : expr) => { # [doc =$ docs] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct $ name { base_id : BaseId , index : usize , } impl $ name { # [inline] fn new (base_id : BaseId , index : usize) -> Self { $ name { base_id , index } } } } ; }
    };
}

define_id!();