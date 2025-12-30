// Generated macro for test (module)
macro_rules! Depcrate_stats_univariate_resamplestest {
() => {
// Module: crate::stats::univariate::resamples
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use quickcheck :: quickcheck ; use quickcheck :: TestResult ; use std :: collections :: HashSet ; use crate :: stats :: univariate :: resamples :: Resamples ; use crate :: stats :: univariate :: Sample ; quickcheck ! { fn subset (size : u8 , nresamples : u8) -> TestResult { let size = size as usize ; let nresamples = nresamples as usize ; if size > 1 { let v : Vec < _ > = (0 .. size) . map (| i | i as f32) . collect () ; let sample = Sample :: new (& v) ; let mut resamples = Resamples :: new (sample) ; let sample = v . iter () . map (|& x | x as i64) . collect ::< HashSet < _ >> () ; TestResult :: from_bool ((0 .. nresamples) . all (| _ | { let resample = resamples . next () . iter () . map (|& x | x as i64) . collect ::< HashSet < _ >> () ; resample . is_subset (& sample) })) } else { TestResult :: discard () } } } # [test] fn different_subsets () { let size = 1000 ; let v : Vec < _ > = (0 .. size) . map (| i | i as f32) . collect () ; let sample = Sample :: new (& v) ; let mut resamples = Resamples :: new (sample) ; let mut num_duplicated = 0 ; for _ in 0 .. 1000 { let sample_1 = resamples . next () . iter () . cloned () . collect :: < Vec < _ > > () ; let sample_2 = resamples . next () . iter () . cloned () . collect :: < Vec < _ > > () ; if sample_1 == sample_2 { num_duplicated += 1 ; } } if num_duplicated > 1 { panic ! ("Found {} duplicate samples" , num_duplicated) ; } } }
};
}
