macro_rules! deps {
    () => {
        Ready!();
        FnOnce1!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < Fut , F , T > Future for Map < Fut , F > where Fut : Future , F : FnOnce1 < Fut :: Output , Output = T > , { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { match self . as_mut () . project () { MapProj :: Incomplete { future , .. } => { let output = ready ! (future . poll (cx)) ; match self . project_replace (Self :: Complete) { MapProjReplace :: Incomplete { f , .. } => Poll :: Ready (f . call_once (output)) , MapProjReplace :: Complete => unreachable ! () , } } MapProj :: Complete => { panic ! ("Map must not be polled after it returned `Poll::Ready`") } } } }
    };
}

impl_55!();