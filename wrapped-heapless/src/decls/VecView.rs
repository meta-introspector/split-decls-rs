macro_rules! deps {
    () => {
        VecInner!();
        Vec!();
    };
}

macro_rules! VecView {
    () => {
        deps!();
        # [doc = " A [`Vec`] with dynamic capacity"] # [doc = ""] # [doc = " [`Vec`] coerces to `VecView`. `VecView` is `!Sized`, meaning it can only ever be used by"] # [doc = " reference."] # [doc = ""] # [doc = " Unlike [`Vec`], `VecView` does not have an `N` const-generic parameter."] # [doc = " This has the ergonomic advantage of making it possible to use functions without needing to know"] # [doc = " at compile-time the size of the buffers used, for example for use in `dyn` traits."] # [doc = ""] # [doc = " `VecView<T>` is to `Vec<T, N>` what `[T]` is to `[T; N]`."] # [doc = ""] # [doc = " ```rust"] # [doc = " use heapless::{Vec, VecView};"] # [doc = ""] # [doc = " let mut vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();"] # [doc = " let view: &VecView<_, _> = &vec;"] # [doc = " assert_eq!(view, &[1, 2, 3, 4]);"] # [doc = ""] # [doc = " let mut_view: &mut VecView<_, _> = &mut vec;"] # [doc = " mut_view.push(5);"] # [doc = " assert_eq!(vec, [1, 2, 3, 4, 5]);"] # [doc = " ```"] pub type VecView < T , LenT = usize > = VecInner < T , LenT , ViewVecStorage < T > > ;
    };
}

VecView!()