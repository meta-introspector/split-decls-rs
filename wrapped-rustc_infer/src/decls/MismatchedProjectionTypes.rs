macro_rules! MismatchedProjectionTypes {
    () => {
        # [derive (Clone)] pub struct MismatchedProjectionTypes < 'tcx > { pub err : ty :: error :: TypeError < 'tcx > , }
    };
}

MismatchedProjectionTypes!()