macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! ACTIONS {
    () => {
        deps!();
        const ACTIONS : [Action ; 16] = [Action :: Nop , Action :: Clear , Action :: Collect , Action :: CsiDispatch , Action :: EscDispatch , Action :: Execute , Action :: Hook , Action :: Ignore , Action :: OscEnd , Action :: OscPut , Action :: OscStart , Action :: Param , Action :: Print , Action :: Put , Action :: Unhook , Action :: BeginUtf8 ,] ;
    };
}

ACTIONS!()