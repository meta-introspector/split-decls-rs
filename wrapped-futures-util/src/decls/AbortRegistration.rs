macro_rules! deps {
    () => {
        AbortInner!();
    };
}

macro_rules! AbortRegistration {
    () => {
        deps!();
        # [doc = " A registration handle for an `Abortable` task."] # [doc = " Values of this type can be acquired from `AbortHandle::new` and are used"] # [doc = " in calls to `Abortable::new`."] # [derive (Debug)] pub struct AbortRegistration { pub (crate) inner : Arc < AbortInner > , }
    };
}

AbortRegistration!()