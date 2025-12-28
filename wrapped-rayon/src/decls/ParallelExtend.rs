macro_rules! deps {
    () => {
        ParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! ParallelExtend {
    () => {
        deps!();
        # [doc = " `ParallelExtend` extends an existing collection with items from a [`ParallelIterator`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Implementing `ParallelExtend` for your type:"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " struct BlackHole {"] # [doc = "     mass: usize,"] # [doc = " }"] # [doc = ""] # [doc = " impl<T: Send> ParallelExtend<T> for BlackHole {"] # [doc = "     fn par_extend<I>(&mut self, par_iter: I)"] # [doc = "         where I: IntoParallelIterator<Item = T>"] # [doc = "     {"] # [doc = "         let par_iter = par_iter.into_par_iter();"] # [doc = "         self.mass += par_iter.count() * size_of::<T>();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let mut bh = BlackHole { mass: 0 };"] # [doc = " bh.par_extend(0i32..1000);"] # [doc = " assert_eq!(bh.mass, 4000);"] # [doc = " bh.par_extend(0i64..10);"] # [doc = " assert_eq!(bh.mass, 4080);"] # [doc = " ```"] pub trait ParallelExtend < T > where T : Send , { # [doc = " Extends an instance of the collection with the elements drawn"] # [doc = " from the parallel iterator `par_iter`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let mut vec = vec![];"] # [doc = " vec.par_extend(0..5);"] # [doc = " vec.par_extend((0..5).into_par_iter().map(|i| i * i));"] # [doc = " assert_eq!(vec, [0, 1, 2, 3, 4, 0, 1, 4, 9, 16]);"] # [doc = " ```"] fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > ; }
    };
}

ParallelExtend!()