macro_rules! NeedsDrop {
    () => {
        # [doc = " Constant containing an ADT that implements `Drop`."] # [doc = " This must be ruled out because implicit promotion would remove side-effects"] # [doc = " that occur as part of dropping that value. N.B., the implicit promotion has"] # [doc = " to reject const Drop implementations because even if side-effects are ruled"] # [doc = " out through other means, the execution of the drop could diverge."] pub struct NeedsDrop ;
    };
}

NeedsDrop!();