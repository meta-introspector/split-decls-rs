macro_rules! deps {
    () => {
        AddToSllResult!();
        Elem!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [cold] pub (crate) fn init < 'a , E : Elem > (head : Option < & 'a Cell < * const E > > , elem : & E ,) -> AddToSllResult < 'a , E > { if let Some (head) = head { link (head , elem) } else { AddToSllResult :: NoHead } }
    };
}

init!()