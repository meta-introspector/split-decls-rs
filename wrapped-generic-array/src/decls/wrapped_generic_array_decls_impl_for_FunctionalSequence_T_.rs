use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, N: ArrayLength> FunctionalSequence<T> for GenericArray<T, N>
where
    Self: GenericSequence<T, Item = T, Length = N>,
{
    #[inline(always)]
    fn map<U, F>(self, mut f: F) -> MappedSequence<Self, T, U>
    where
        Self: MappedGenericSequence<T, U>,
        F: FnMut(T) -> U,
    {
        unsafe {
            let mut array = ManuallyDrop::new(self);
            let mut source = IntrusiveArrayConsumer::new(&mut array);
            let (array_iter, position) = source.iter_position();
            FromIterator::from_iter(array_iter.map(|src| {
                let value = ptr::read(src);
                *position += 1;
                f(value)
            }))
        }
    }
    #[inline(always)]
    fn try_map<U, F, E>(self, mut f: F) -> Result<MappedSequence<Self, T, U>, E>
    where
        Self: MappedGenericSequence<T, U>,
        Mapped<Self, T, U>: FallibleGenericSequence<U>,
        F: FnMut(Self::Item) -> Result<U, E>,
    {
        unsafe {
            let mut array = ManuallyDrop::new(self);
            let mut source = IntrusiveArrayConsumer::new(&mut array);
            let (array_iter, position) = source.iter_position();
            <Mapped<Self, T, U> as FallibleGenericSequence<U>>::from_fallible_iter(array_iter.map(
                |src| {
                    let value = ptr::read(src);
                    *position += 1;
                    f(value)
                },
            ))
        }
    }
    #[inline(always)]
    fn zip<B, Rhs, U, F>(self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>
    where
        Self: MappedGenericSequence<T, U>,
        Rhs: MappedGenericSequence<B, U, Mapped = MappedSequence<Self, T, U>>,
        Rhs: GenericSequence<B, Length = Self::Length>,
        F: FnMut(T, Rhs::Item) -> U,
    {
        rhs.inverted_zip(self, f)
    }
    #[inline(always)]
    fn fold<U, F>(self, init: U, mut f: F) -> U
    where
        F: FnMut(U, T) -> U,
    {
        unsafe {
            let mut array = ManuallyDrop::new(self);
            let mut source = IntrusiveArrayConsumer::new(&mut array);
            let (array_iter, position) = source.iter_position();
            array_iter.fold(init, |acc, src| {
                let value = ptr::read(src);
                *position += 1;
                f(acc, value)
            })
        }
    }
    #[inline(always)]
    fn try_fold<U, E, F>(self, init: U, mut f: F) -> Result<U, E>
    where
        F: FnMut(U, Self::Item) -> Result<U, E>,
    {
        unsafe {
            let mut array = ManuallyDrop::new(self);
            let mut source = IntrusiveArrayConsumer::new(&mut array);
            let (mut array_iter, position) = source.iter_position();
            array_iter.try_fold(init, |acc, src| {
                let value = ptr::read(src);
                *position += 1;
                f(acc, value)
            })
        }
    }
}
