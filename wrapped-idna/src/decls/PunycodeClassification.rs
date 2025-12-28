macro_rules! PunycodeClassification {
    () => {
        # [derive (PartialEq , Eq , Copy , Clone)] enum PunycodeClassification { Ascii , Unicode , Error , }
    };
}

PunycodeClassification!()