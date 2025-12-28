macro_rules! ConstConditionsHold {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ConstConditionsHold { Yes , No , }
    };
}

ConstConditionsHold!()