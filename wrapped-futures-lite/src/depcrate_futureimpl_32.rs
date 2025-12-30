// Generated macro for impl_32 (impl)
macro_rules! Depcrate_futureimpl_32 {
() => {
// Module: crate::future
// Provides: {"impl_32"}
// Dependencies: {}
impl < T1 , T2 , E , F1 , F2 > Future for TryZip < F1 , T1 , F2 , T2 > where F1 : Future < Output = Result < T1 , E > > , F2 : Future < Output = Result < T2 , E > > , { type Output = Result < (T1 , T2) , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; if let Some (future) = this . future1 . as_mut () . as_pin_mut () { if let Poll :: Ready (out) = future . poll (cx) { match out { Ok (t) => { * this . output1 = Some (t) ; this . future1 . set (None) ; } Err (err) => return Poll :: Ready (Err (err)) , } } } if let Some (future) = this . future2 . as_mut () . as_pin_mut () { if let Poll :: Ready (out) = future . poll (cx) { match out { Ok (t) => { * this . output2 = Some (t) ; this . future2 . set (None) ; } Err (err) => return Poll :: Ready (Err (err)) , } } } take_zip_from_parts (this . output1 , this . output2) . map (Ok) } }
};
}
