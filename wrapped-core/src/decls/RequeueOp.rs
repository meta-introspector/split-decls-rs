macro_rules! RequeueOp {
    () => {
        # [doc = " Operation that `unpark_requeue` should perform."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum RequeueOp { # [doc = " Abort the operation without doing anything."] Abort , # [doc = " Unpark one thread and requeue the rest onto the target queue."] UnparkOneRequeueRest , # [doc = " Requeue all threads onto the target queue."] RequeueAll , # [doc = " Unpark one thread and leave the rest parked. No requeuing is done."] UnparkOne , # [doc = " Requeue one thread and leave the rest parked on the original queue."] RequeueOne , }
    };
}

RequeueOp!()