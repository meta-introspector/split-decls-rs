macro_rules! deps {
    () => {
        FieldValue!();
        BoxResolveFut!();
    };
}

macro_rules! FieldFuture {
    () => {
        deps!();
        # [doc = " A future that returned from field resolver"] pub enum FieldFuture < 'a > { # [doc = " A pure value without any async operation"] Value (Option < FieldValue < 'a > >) , # [doc = " A future that returned from field resolver"] Future (BoxResolveFut < 'a >) , }
    };
}

FieldFuture!()