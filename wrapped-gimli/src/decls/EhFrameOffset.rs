macro_rules! EhFrameOffset {
    () => {
        # [doc = " An offset into the `.eh_frame` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct EhFrameOffset < T = usize > (pub T) ;
    };
}

EhFrameOffset!();