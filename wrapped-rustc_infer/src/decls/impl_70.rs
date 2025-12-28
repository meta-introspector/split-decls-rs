macro_rules! deps {
    () => {
        LexicalRegionResolutions!();
        VarValue!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'tcx > LexicalRegionResolutions < 'tcx > { fn normalize < T > (& self , tcx : TyCtxt < 'tcx > , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (tcx , value , | r , _db | self . resolve_region (tcx , r)) } fn value (& self , rid : RegionVid) -> & VarValue < 'tcx > { & self . values [rid] } fn value_mut (& mut self , rid : RegionVid) -> & mut VarValue < 'tcx > { & mut self . values [rid] } pub (crate) fn resolve_region (& self , tcx : TyCtxt < 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { let result = match r . kind () { ty :: ReVar (rid) => match self . values [rid] { VarValue :: Empty (_) => r , VarValue :: Value (r) => r , VarValue :: ErrorValue => tcx . lifetimes . re_static , } , _ => r , } ; debug ! ("resolve_region({:?}) = {:?}" , r , result) ; result } }
    };
}

impl_70!();