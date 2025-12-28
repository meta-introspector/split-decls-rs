macro_rules! deps {
    () => {
        Elem!();
        AddToSllResult!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [cold] pub (crate) fn init < 'a , E : Elem > (head : Option < & 'a Cell < * const E > > , elem : & E ,) -> AddToSllResult < 'a , E > { if let Some (head) = head { link (head , elem) } else { AddToSllResult :: NoHead } }
    };
}

init!();