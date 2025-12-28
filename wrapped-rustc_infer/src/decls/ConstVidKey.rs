macro_rules! ConstVidKey {
    () => {
        # [derive (PartialEq , Copy , Clone , Debug)] pub (crate) struct ConstVidKey < 'tcx > { pub vid : ty :: ConstVid , pub phantom : PhantomData < ty :: Const < 'tcx > > , }
    };
}

ConstVidKey!()