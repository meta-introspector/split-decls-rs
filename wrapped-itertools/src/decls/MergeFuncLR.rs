macro_rules! MergeFuncLR {
    () => {
        # [derive (Clone , Debug)] pub struct MergeFuncLR < F , T > (F , PhantomData < T >) ;
    };
}

MergeFuncLR!()