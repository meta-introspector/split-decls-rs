macro_rules! MergedObject {
    () => {
        # [doc (hidden)] pub struct MergedObject < A , B > (pub A , pub B) ;
    };
}

MergedObject!()